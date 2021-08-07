package thm.mic.ser.user_service.repository;

import thm.mic.ser.user_service.dto.AppUser;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.stereotype.Repository;

import javax.transaction.Transactional;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Repository
public interface AppUserRepository extends JpaRepository<AppUser, Integer> {

    @Query(value = "SELECT * FROM app_user WHERE guid = ?1", nativeQuery = true)
    Optional<AppUser> getByUserGUID(String uuid);

    @Query(value = "SELECT * FROM app_user WHERE email = ?1", nativeQuery = true)
    Optional<AppUser> getByEmail(String uuid);
}
